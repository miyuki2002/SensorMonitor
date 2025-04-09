# Tài liệu Ứng dụng Android Giám sát Cảm biến ESP32 

## 1. Giới thiệu

Ứng dụng Android Giám sát Cảm biến ESP32 là một ứng dụng di động được thiết kế để theo dõi và hiển thị dữ liệu từ nhiều cảm biến được kết nối với một thiết bị ESP32. Ứng dụng này cung cấp giao diện người dùng trực quan để xem các chỉ số cảm biến khác nhau như nhiệt độ, độ ẩm, mực nước, pH, độ mặn, mưa và độ ẩm đất.

## 2. Cấu trúc Ứng dụng

Ứng dụng được tổ chức theo mô hình MVVM (Model-View-ViewModel) và sử dụng các thành phần kiến trúc Android hiện đại. Các thành phần chính bao gồm:

- **Model**: Đại diện cho dữ liệu và logic nghiệp vụ.
  - `ESP32SensorData`: Lớp chứa dữ liệu cảm biến từ ESP32.
  - `SensorReading`: Lớp đại diện cho một bản ghi cảm biến trong cơ sở dữ liệu.

- **Repository**: Xử lý việc truy xuất và lưu trữ dữ liệu.
  - `SensorRepository`: Quản lý việc lấy dữ liệu từ API ESP32 và tương tác với cơ sở dữ liệu local.

- **ViewModel**: Xử lý logic giao diện người dùng và giữ trạng thái.

- **View**: Các thành phần giao diện người dùng.

## 3. Tính năng Chính

- Lấy dữ liệu cảm biến mới nhất từ ESP32.
- Lưu trữ dữ liệu cảm biến vào cơ sở dữ liệu local.
- Hiển thị dữ liệu cảm biến mới nhất cho từng loại cảm biến.
- Xem lịch sử dữ liệu cảm biến trong một khoảng thời gian.
- Quản lý nhiều loại cảm biến khác nhau.
- Xóa dữ liệu cũ để tối ưu hóa bộ nhớ.

## 4. Kết nối với ESP32

Ứng dụng kết nối với ESP32 thông qua một API RESTful. Địa chỉ IP của ESP32 được lưu trữ trong SharedPreferences và có thể được cấu hình bởi người dùng.

## 5. Xử lý Dữ liệu Cảm biến

- Ứng dụng sử dụng Retrofit để gọi API ESP32 và lấy dữ liệu cảm biến mới nhất.
- Dữ liệu nhận được được lưu vào cơ sở dữ liệu local sử dụng Room.
- LiveData được sử dụng để cập nhật giao diện người dùng khi có dữ liệu mới.

## 6. Bảo mật và Quyền riêng tư

- Ứng dụng yêu cầu quyền truy cập internet để kết nối với ESP32.
- Dữ liệu cảm biến được lưu trữ cục bộ trên thiết bị.

## 7. Yêu cầu Hệ thống

- Android 5.0 (API level 21) trở lên.
- Kết nối internet để giao tiếp với ESP32.

## 8. Cấu hình Firebase

Để tích hợp Firebase vào ứng dụng SensorMonitor, hãy làm theo các bước sau:

1. **Tạo dự án Firebase:**
   - Truy cập [Firebase Console](https://console.firebase.google.com/).
   - Nhấp vào "Add project" và đặt tên cho dự án của bạn.
   - Làm theo hướng dẫn để tạo dự án mới.

2. **Thêm ứng dụng Android vào dự án Firebase:**
   - Trong Firebase Console, chọn dự án của bạn.
   - Nhấp vào biểu tượng Android để thêm ứng dụng Android.
   - Nhập package name của ứng dụng (com.example.sensormonitor).
   - Tải xuống file `google-services.json`.

3. **Thêm Firebase SDK vào ứng dụng:**
   - Đặt file `google-services.json` vào thư mục `app/` của dự án Android.
   - Thêm plugin Google Services vào file `build.gradle` cấp dự án:
     ```gradle
     buildscript {
         dependencies {
             classpath 'com.google.gms:google-services:4.3.15'
         }
     }
     ```
   - Áp dụng plugin trong file `build.gradle` cấp module (app):
     ```gradle
     apply plugin: 'com.google.gms.google-services'
     ```

4. **Thêm các phụ thuộc Firebase:**
   - Trong file `build.gradle` cấp module (app), thêm các phụ thuộc cần thiết:
     ```gradle
     dependencies {
         implementation platform('com.google.firebase:firebase-bom:32.1.0')
         implementation 'com.google.firebase:firebase-analytics'
         implementation 'com.google.firebase:firebase-database'
         // Thêm các phụ thuộc khác nếu cần
     }
     ```

5. **Khởi tạo Firebase trong ứng dụng:**
   - Trong `MainActivity` hoặc `Application` class, thêm mã để khởi tạo Firebase:
     ```java
     import com.google.firebase.FirebaseApp;

     public class MainActivity extends AppCompatActivity {
         @Override
         protected void onCreate(Bundle savedInstanceState) {
             super.onCreate(savedInstanceState);
             FirebaseApp.initializeApp(this);
             // ... code khác
         }
     }
     ```

6. **Sử dụng Firebase trong ứng dụng:**
   - Bây giờ bạn có thể sử dụng các tính năng của Firebase như Realtime Database, Authentication, Cloud Messaging, v.v.
   - Ví dụ, để sử dụng Realtime Database:
     ```java
     import com.google.firebase.database.FirebaseDatabase;
     import com.google.firebase.database.DatabaseReference;

     DatabaseReference database = FirebaseDatabase.getInstance().getReference();
     database.child("sensors").setValue(sensorData);
     ```

7. **Cấu hình quy tắc bảo mật:**
   - Trong Firebase Console, đi đến phần "Realtime Database" hoặc "Firestore".
   - Cấu hình quy tắc bảo mật để kiểm soát quyền truy cập dữ liệu.

8. **Kiểm tra kết nối:**
   - Chạy ứng dụng và kiểm tra xem dữ liệu có được gửi đến Firebase không.
   - Kiểm tra Firebase Console để xem dữ liệu được lưu trữ.

Lưu ý: Đảm bảo cập nhật các phiên bản SDK và plugin mới nhất khi cấu hình. Tham khảo [tài liệu chính thức của Firebase](https://firebase.google.com/docs/android/setup) để biết thêm chi tiết và cập nhật mới nhất.

## 8. Hướng dẫn Build Source thành APK

Để build source code của ứng dụng SensorMonitor thành file APK, hãy làm theo các bước sau:

1. **Chuẩn bị môi trường:**
   - Đảm bảo bạn đã cài đặt Android Studio phiên bản mới nhất.
   - Cập nhật Android SDK Tools và Android SDK Build-Tools lên phiên bản mới nhất thông qua SDK Manager trong Android Studio.

2. **Mở dự án trong Android Studio:**
   - Khởi động Android Studio.
   - Chọn "Open an existing Android Studio project".
   - Duyệt đến thư mục chứa source code của SensorMonitor và mở nó.

3. **Cấu hình Gradle:**
   - Mở file `app/build.gradle`.
   - Kiểm tra và cập nhật các phiên bản dependencies nếu cần.
   - Đảm bảo `applicationId`, `versionCode`, và `versionName` đã được cấu hình đúng.

4. **Chọn Build Variant:**
   - Trong Android Studio, mở tab "Build Variants" (thường nằm ở góc trái dưới).
   - Chọn "release" làm Active Build Variant.

5. **Tạo Signed APK:**
   - Trong menu, chọn Build > Generate Signed Bundle / APK.
   - Chọn APK và nhấn Next.
   - Nếu bạn chưa có keystore, tạo một keystore mới. Nếu đã có, chọn keystore hiện có.
   - Điền thông tin key alias và mật khẩu.
   - Chọn thư mục đích để lưu APK.
   - Chọn build type là "release".
   - Nhấn Finish để bắt đầu quá trình build.

6. **Tối ưu hóa APK (tùy chọn):**
   - Trong file `app/build.gradle`, đảm bảo `minifyEnabled` và `shrinkResources` được đặt thành `true` trong `buildTypes.release`.
   - Kiểm tra file `proguard-rules.pro` để đảm bảo các quy tắc ProGuard phù hợp đã được cấu hình.

7. **Kiểm tra APK:**
   - Sau khi quá trình build hoàn tất, bạn sẽ tìm thấy file APK trong thư mục `app/build/outputs/apk/release/`.
   - Cài đặt APK này trên một thiết bị Android để kiểm tra.

8. **Phân phối APK:**
   - Bạn có thể phân phối APK này trực tiếp hoặc tải lên Google Play Store (yêu cầu tài khoản nhà phát triển).

Lưu ý: 
- Đảm bảo bạn giữ keystore và mật khẩu an toàn. Bạn sẽ cần chúng để cập nhật ứng dụng trong tương lai.
- Nếu bạn gặp bất kỳ lỗi nào trong quá trình build, hãy kiểm tra log lỗi trong Android Studio để biết thêm chi tiết.

---
