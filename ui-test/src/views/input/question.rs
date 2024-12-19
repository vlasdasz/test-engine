use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    ui::{Question, UI, view},
    ui_test::{check_colors, inject_touches},
    wait_for_next_frame,
};
use tokio::{spawn, sync::oneshot::channel};

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

    check_colors(
        r#"
              88  395 -  25  51  76
             121  395 - 255 255 255
             138  395 - 255 255 255
             187  389 - 203 203 243
             215  390 -   1   1 203
             230  390 - 255 255 255
             271  390 - 255 255 255
             319  384 - 255 255 255
             362  384 - 255 255 255
             382  384 - 255 255 255
             457  385 - 255 255 255
             482  385 - 255 255 255
             503  389 -  25  51  76
             503  154 -  25  51  76
             442  196 - 255 255 255
             349  228 - 255 255 255
             335  240 - 255 255 255
             328  244 - 255 255 255
             302  260 - 255 255 255
             279  256 -   1   1   1
             259  256 -   0   0   0
             257  256 -   0   0   0
             257  256 -   0   0   0
             231  256 - 255 255 255
             134  256 - 255 255 255
              71  256 -  25  51  76
             137  114 -  25  51  76
             226  197 - 255 255 255
             254  251 -   0   0   0
             271  251 -   0   0   0
             315  251 - 239 239 239
             384  383 - 255 255 255
             211  396 - 255 255 255
             186  390 - 255 255 255
             224  390 -  19  19 207
             224  390 -  19  19 207
             326  255 -   0   0   0
             300  354 - 255 255 255
             300  329 - 255 255 255
        "#,
    )
    .await?;

    inject_touches(
        "
            220  388  b
            220  388  e
        ",
    )
    .await;

    assert_eq!(rc.await.unwrap(), false);

    let (se, rc) = channel::<bool>();

    from_main(|| {
        Question::ask("Hello?")
            .options("left", "right")
            .callback(|answer| se.send(answer).unwrap());
    })
    .await;

    check_colors(
        r#"
              96  393 -  25  51  76
             206  393 -   1   1 203
             386  388 - 255 255 255
             521  342 -  25  51  76
             342  255 -  27  27  27
             250  254 - 255 255 255
             250  141 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        "
            417  392  b
            417  392  e
        ",
    )
    .await;

    assert_eq!(rc.await.unwrap(), true);

    let a = spawn(async {
        let val = Question::ask("Hello?").options("left", "right").await;
        assert_eq!(val, false);
    });

    wait_for_next_frame().await;

    inject_touches(
        "
            220  388  b
            220  388  e
        ",
    )
    .await;

    a.await.unwrap();

    wait_for_next_frame().await;

    let a = spawn(async {
        let val = Question::ask("Hello?").options("left", "right").await;
        assert_eq!(val, true);
    });

    wait_for_next_frame().await;
    wait_for_next_frame().await;

    inject_touches(
        "
            417  392  b
            417  392  e
        ",
    )
    .await;

    a.await.unwrap();

    debug!("Question: OK");

    Ok(())
}
