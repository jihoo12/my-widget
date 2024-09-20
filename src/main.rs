use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};
use gtk_layer_shell_rs::LayerSurface;

fn activate(app: &Application) {
    // 새 GTK 애플리케이션 창 생성
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(300)
        .default_height(300)
        .build();

    // 레이어 셸 초기화
    LayerSurface::init_for_window(&window);

    // 레이블 위젯 생성 및 텍스트 설정
    let label = Label::new(Some("Hello, World!"));

    // 창에 레이블 추가
    window.set_child(Some(&label));

    // 창 표시
    window.present();
}

fn main() {
    // GTK 애플리케이션 생성
    let app = Application::builder()
        .application_id("com.example.GtkLayerShellApp")
        .build();

    // 활성화 신호 연결
    app.connect_activate(activate);

    // 애플리케이션 실행 및 상태 저장
    let status = app.run();

    std::process::exit(status);
}