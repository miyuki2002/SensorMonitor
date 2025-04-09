# Ứng dụng Android Giám sát Cảm biến ESP32

## Giới thiệu

Ứng dụng Android Giám sát Cảm biến ESP32 là một ứng dụng di động được thiết kế để theo dõi và hiển thị dữ liệu từ nhiều cảm biến được kết nối với một thiết bị ESP32. Ứng dụng này cung cấp giao diện người dùng trực quan để xem các chỉ số cảm biến khác nhau như nhiệt độ, độ ẩm, mực nước, pH, độ mặn, mưa và độ ẩm đất.

## Tính năng chính

- Lấy dữ liệu cảm biến mới nhất từ ESP32
- Lưu trữ dữ liệu cảm biến vào cơ sở dữ liệu local
- Hiển thị dữ liệu cảm biến mới nhất cho từng loại cảm biến
- Xem lịch sử dữ liệu cảm biến trong một khoảng thời gian
- Quản lý nhiều loại cảm biến khác nhau
- Xóa dữ liệu cũ để tối ưu hóa bộ nhớ

## Yêu cầu hệ thống

- Android 5.0 (API level 21) trở lên
- Kết nối internet để giao tiếp với ESP32

## Cài đặt

1. Tải xuống file APK từ phần Releases của repository này.
2. Cài đặt APK trên thiết bị Android của bạn.
3. Cấp quyền truy cập internet cho ứng dụng nếu được yêu cầu.

## Cấu hình

1. Mở ứng dụng và điều hướng đến phần cài đặt.
2. Nhập địa chỉ IP của thiết bị ESP32 của bạn.
3. Lưu cài đặt và khởi động lại ứng dụng nếu cần.

## Sử dụng

- Màn hình chính hiển thị dữ liệu cảm biến mới nhất.
- Sử dụng menu để điều hướng giữa các chức năng khác nhau của ứng dụng.
- Xem biểu đồ và lịch sử dữ liệu bằng cách chọn loại cảm biến và khoảng thời gian mong muốn.

## Phát triển

Nếu bạn muốn đóng góp vào dự án hoặc tự build ứng dụng, hãy làm theo các bước sau:

1. Clone repository này.
2. Mở dự án trong Android Studio.
3. Cấu hình Firebase theo hướng dẫn trong tài liệu.
4. Build và chạy ứng dụng trên thiết bị hoặc máy ảo Android.

## Đóng góp

Chúng tôi hoan nghênh mọi đóng góp cho dự án. Nếu bạn muốn đóng góp, vui lòng:

1. Fork repository
2. Tạo branch mới (`git checkout -b feature/AmazingFeature`)
3. Commit các thay đổi của bạn (`git commit -m 'Add some AmazingFeature'`)
4. Push lên branch (`git push origin feature/AmazingFeature`)
5. Mở một Pull Request

## Giấy phép

Dự án này được phân phối dưới giấy phép MIT. Xem file `LICENSE` để biết thêm thông tin.

## Liên hệ

Nếu bạn có bất kỳ câu hỏi hoặc đề xuất nào, vui lòng mở một issue trong repository này hoặc liên hệ trực tiếp với chúng tôi qua email: [hoanghcm188@gmail.com]