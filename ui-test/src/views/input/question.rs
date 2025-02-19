use std::time::Duration;

use anyhow::Result;
use test_engine::{
    from_main,
    ui::{Question, UI, view},
    ui_test::inject_touches,
    wait_for_next_frame,
};
use tokio::{spawn, sync::oneshot::channel, time::sleep};

#[view]
struct QuestionTestView {}

pub async fn test_question() -> Result<()> {
    let _view = UI::init_test_view::<QuestionTestView>().await;

    let (se, rc) = channel::<bool>();

    from_main(|| {
        Question::ask("Hello?")
            .options("left", "right")
            .callback(|answer| se.send(answer).unwrap());
    })
    .await;

    inject_touches(
        "
            236  398  b
            236  397  e

        ",
    )
    .await;

    assert_eq!(rc.await?, false);

    let (se, rc) = channel::<bool>();

    from_main(|| {
        Question::ask("Hello?")
            .options("left", "right")
            .callback(|answer| se.send(answer).unwrap());
    })
    .await;

    wait_for_next_frame().await;
    wait_for_next_frame().await;

    inject_touches(
        "
            378  396  b
            378  396  e

        ",
    )
    .await;

    assert_eq!(rc.await?, true);

    let a = spawn(async {
        let val = Question::ask("Hello?").options("left", "right").await;
        assert_eq!(val, false);
    });

    wait_for_next_frame().await;
    wait_for_next_frame().await;

    inject_touches(
        "
            220  388  b
            220  388  e
        ",
    )
    .await;

    a.await?;

    wait_for_next_frame().await;

    let a = spawn(async {
        let val = Question::ask("Hello?").options("left", "right").await;
        assert_eq!(val, true);
    });

    sleep(Duration::from_secs_f32(0.1)).await;

    inject_touches(
        "
            402  387  b
            402  387  e
        ",
    )
    .await;

    a.await?;

    Ok(())
}
