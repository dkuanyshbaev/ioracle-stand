use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};

const LEDS_IN_LINE: i32 = 144;
const YAO_PIN: i32 = 12;
const LI_PIN: i32 = 13;
const YAO_COLOUR: (u8, u8, u8) = (51, 0, 180);
const LI_COLOUR: (u8, u8, u8) = (230, 4, 211);

type LEDResult<T> = Result<T, WS2811Error>;

fn build_controller(brightness: u8) -> LEDResult<Controller> {
    ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(YAO_PIN)
                .count(6 * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(brightness)
                .build(),
        )
        .channel(
            1,
            ChannelBuilder::new()
                .pin(LI_PIN)
                .count(3 * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(brightness)
                .build(),
        )
        .build()
}

fn main() -> LEDResult<()> {
    println!("IORACLE Stand Mode");
    let mut controller = build_controller(50)?;

    let yao = controller.leds_mut(0);
    for num in 0..yao.len() {
        // c, a, b
        yao[num as usize] = [YAO_COLOUR.2, YAO_COLOUR.0, YAO_COLOUR.1, 0];
    }

    let li = controller.leds_mut(1);
    for num in 0..li.len() {
        // c, b, a
        li[num as usize] = [LI_COLOUR.2, LI_COLOUR.1, LI_COLOUR.0, 0];
    }

    controller.render()
}
