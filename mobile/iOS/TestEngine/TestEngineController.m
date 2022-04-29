//
//  TestEngineController.m
//  TestEngine
//
//  Created by Vladas Zakrevskis on 26.03.2022.
//

#import <GLKit/GLKit.h>
#import <OpenGLES/ES3/gl.h>
#import <CoreMotion/CoreMotion.h>

#import "TestEngineController.h"

#import "test_engine.h"



@interface TestEngineController ()
@property (nonatomic) CMMotionManager* motion;
@property (nonatomic) NSTimer* timer;
@end

@implementation TestEngineController

- (void)viewDidLoad {
    [super viewDidLoad];
    [self setup];
    
    CGRect screen = [[UIScreen mainScreen] bounds];
    
    set_monitor(326,
                [UIScreen mainScreen].scale,
                60,
                screen.size.width,
                screen.size.height,
                300,
                300,
                7);
}

- (void)viewDidLayoutSubviews {
    [super viewDidLayoutSubviews];
    set_screen_size(self.view.frame.size.width, self.view.frame.size.height);
}

- (void)update {
    CMAttitude *gyro = self.motion.deviceMotion.attitude;
    if (gyro != nil) {
        set_gyro(gyro.pitch, gyro.roll, gyro.yaw);
    }
    
    update_screen();
}

- (void)setup {
    
    self.preferredFramesPerSecond = 240; 
    
    EAGLContext* context = [[EAGLContext alloc] initWithAPI:kEAGLRenderingAPIOpenGLES3];
    
    if (context == nil) {
        NSAssert(false, @"Failed to initialize OpenGL");
    } else {
        NSLog(@"%s", "kEAGLRenderingAPIOpenGLES3");
    }
    
    NSLog(@"%@", context);
    [EAGLContext setCurrentContext:context];
    GLKView* view = (GLKView*)self.view;
    view.context = context;
    view.drawableColorFormat = GLKViewDrawableColorFormatRGBA8888;
    view.drawableDepthFormat = GLKViewDrawableDepthFormat16;
    view.drawableStencilFormat = GLKViewDrawableStencilFormat8;
    view.multipleTouchEnabled = true;
    
    [self startGyro];
}

- (void)startGyro {
    self.motion = [CMMotionManager new];
    
    if (!self.motion.isGyroAvailable) {
        return;
    }
    
    self.motion.gyroUpdateInterval = 1.0 / 60.0;
    [self.motion startGyroUpdates];
    [self.motion startDeviceMotionUpdates];

//
//    if motion.isGyroAvailable {
//               self.motion.gyroUpdateInterval = 1.0 / 60.0
//               self.motion.startGyroUpdates()
//
//               // Configure a timer to fetch the accelerometer data.
//
//               self.timer = Timer(fire: Date(), interval: (1.0/60.0),
//
//               repeats: true, block: { (timer) in
//   // Get the gyro data.
//                   if let data = self.motion.gyroData {
//                       let x = data.rotationRate.x
//                       let y = data.rotationRate.y
//                       let z = data.rotationRate.z
//                       print("gyro works")
//               // Use the gyroscope data in your app.
//                   }
//                   print("outloop")
//               })
//
//         // Add the timer to the current run loop.
//               RunLoop.current.add(self.timer, forMode: RunLoop.Mode.default)
//           }
}

- (void)process_touch:(UITouch*)touch event:(int)event {
    const long long touch_id = (long long)touch;
    const CGPoint ns_location = [touch locationInView: self.view];
    on_touch((int)touch_id, ns_location.x, ns_location.y, event);
}

- (void)touchesBegan:(NSSet<UITouch*>*)touches withEvent:(UIEvent*)event {
    for (UITouch* touch in touches) {
        [self process_touch:touch event:0];
    }
}

- (void)touchesMoved:(NSSet<UITouch*>*)touches withEvent:(UIEvent*)event {
    for (UITouch* touch in touches) {
        [self process_touch:touch event:1];
    }
}

- (void)touchesEnded:(NSSet<UITouch*>*)touches withEvent:(UIEvent*)event {
    for (UITouch* touch in touches) {
        [self process_touch:touch event:2];
    }
}
@end
