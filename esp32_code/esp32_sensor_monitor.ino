#include <WiFi.h>
#include <DHT.h>
#include <FirebaseESP32.h>
#include <ArduinoJson.h>
#include <time.h>

// Thông tin xác thực WiFi
const char* ssid = "YOUR_WIFI_SSID"; // Thay thế bằng SSID WiFi của bạn
const char* password = "YOUR_WIFI_PASSWORD"; // Thay thế bằng mật khẩu WiFi của bạn

// Thông tin xác thực Firebase
#define FIREBASE_HOST "your-project-id.firebaseio.com" // Không có "https://" và "/" ở cuối
#define FIREBASE_AUTH "your-firebase-database-secret"
#define FIREBASE_PATH "/sensor_readings"

// Cảm biến DHT
#define DHTPIN 4
#define DHTTYPE DHT11 // Thay thế bằng loại cảm biến khác, tại thầy m bảo dùng custom nên t đéo biết
DHT dht(DHTPIN, DHTTYPE);

// Cảm biến mức nước
#define WATER_LEVEL_PIN 34

// Cảm biến pH
#define PH_SENSOR_PIN 35

// Cảm biến độ mặn
#define SALINITY_SENSOR_PIN 32

// Cảm biến mưa
#define RAIN_SENSOR_PIN 33

// Cảm biến độ ẩm đất
#define SOIL_MOISTURE_PIN 36

// Đối tượng Firebase
FirebaseData firebaseData;
FirebaseJson sensorJson;

// Cài đặt NTP
const char* ntpServer = "pool.ntp.org";
const long gmtOffset_sec = 0;  // GMT offset in seconds (0 = GMT)
const int daylightOffset_sec = 3600; // Daylight savings time offset (3600 = 1 hour)

// Cài đặt bộ đếm thời gian
unsigned long previousMillis = 0;
const long interval = 60000; // Gửi dữ liệu mỗi 1 phút (60000ms)

void setup() {
  Serial.begin(115200);
  
  // Khởi tạo cảm biến DHT
  dht.begin();
  
  // Kết nối WiFi
  WiFi.begin(ssid, password);
  Serial.println("Đang kết nối WiFi...");
  
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  
  Serial.println("");
  Serial.println("WiFi đã kết nối");
  Serial.println("Địa chỉ IP: ");
  Serial.println(WiFi.localIP());
  
  // Khởi tạo thời gian với NTP
  configTime(gmtOffset_sec, daylightOffset_sec, ntpServer);
  
  // Khởi tạo Firebase
  Firebase.begin(FIREBASE_HOST, FIREBASE_AUTH);
  Firebase.reconnectWiFi(true);
  Firebase.setMaxRetry(firebaseData, 3);
  Firebase.setMaxErrorQueue(firebaseData, 30);
  Firebase.enableClassicRequest(firebaseData, true);
  
  Serial.println("Firebase đã khởi tạo");
}

void loop() {
  unsigned long currentMillis = millis();
  
  // Kiểm tra xem đã đến lúc gửi dữ liệu chưa
  if (currentMillis - previousMillis >= interval) {
    previousMillis = currentMillis;
    
    // Đọc và gửi dữ liệu cảm biến
    sendSensorDataToFirebase();
  }
}

void sendSensorDataToFirebase() {
  // Đọc giá trị cảm biến
  float temperature = dht.readTemperature();
  float humidity = dht.readHumidity();
  float waterLevel = readWaterLevel();
  float ph = readPH();
  float salinity = readSalinity();
  bool rain = detectRain();
  float soilMoisture = readSoilMoisture();
  
  // Lấy timestamp hiện tại
  struct tm timeinfo;
  if (!getLocalTime(&timeinfo)) {
    Serial.println("Không thể lấy thời gian");
  }
  
  time_t now;
  time(&now);
  
  // Xóa dữ liệu trước đó
  sensorJson.clear();
  
  // Thêm giá trị vào JSON
  sensorJson.add("temperature", isnan(temperature) ? 0 : temperature);
  sensorJson.add("humidity", isnan(humidity) ? 0 : humidity);
  sensorJson.add("water_level", waterLevel);
  sensorJson.add("ph", ph);
  sensorJson.add("salinity", salinity);
  sensorJson.add("rain", rain);
  sensorJson.add("soil_moisture", soilMoisture);
  sensorJson.add("timestamp", now * 1000); // Chuyển đổi sang mili giây cho JavaScript
  
  // Tạo khóa duy nhất bằng timestamp
  String dataPath = FIREBASE_PATH;
  dataPath += "/";
  dataPath += String(now);
  
  // Gửi đến Firebase
  if (Firebase.pushJSON(firebaseData, dataPath, sensorJson)) {
    Serial.println("Dữ liệu đã được gửi đến Firebase thành công");
  } else {
    Serial.println("Không thể gửi dữ liệu đến Firebase");
    Serial.println("Lỗi: " + firebaseData.errorReason());
  }
}

float readWaterLevel() {
  // Đọc giá trị analog từ cảm biến mức nước
  int rawValue = analogRead(WATER_LEVEL_PIN);
  // Chuyển đổi sang cm (điều chỉnh dựa trên hiệu chuẩn cảm biến của bạn)
  return map(rawValue, 0, 4095, 0, 100);
}

float readPH() {
  // Đọc giá trị analog từ cảm biến pH
  int rawValue = analogRead(PH_SENSOR_PIN);
  // Chuyển đổi thành pH (điều chỉnh dựa trên hiệu chuẩn cảm biến của bạn)
  float voltage = rawValue * (3.3 / 4095.0);
  return 3.5 * voltage;
}

float readSalinity() {
  // Đọc giá trị analog từ cảm biến độ mặn
  int rawValue = analogRead(SALINITY_SENSOR_PIN);
  // Chuyển đổi thành ppt (phần nghìn) - hiệu chuẩn cho cảm biến của bạn
  return map(rawValue, 0, 4095, 0, 50);
}

bool detectRain() {
  // Đọc giá trị kỹ thuật số từ cảm biến mưa
  return digitalRead(RAIN_SENSOR_PIN) == LOW;
}

float readSoilMoisture() {
  // Đọc giá trị analog từ cảm biến độ ẩm đất
  int rawValue = analogRead(SOIL_MOISTURE_PIN);
  // Chuyển đổi thành phần trăm
  return map(rawValue, 4095, 0, 0, 100);
}
