//
//  TestEngineController.m
//  TestEngine
//
//  Created by Vladas Zakrevskis on 26.03.2022.
//

#import <GLKit/GLKit.h>
#include <OpenGLES/ES3/gl.h>

#import "TestEngineController.h"

#include "test_engine.h"


@interface TestEngineController ()

@end

@implementation TestEngineController

- (void)viewDidLoad {
    [super viewDidLoad];
    [self setup];
    
    CGRect screen = [[UIScreen mainScreen] bounds];

    set_monitor(326,
                [UIScreen mainScreen].scale,
                60,
                self.view.frame.size.width,
                self.view.frame.size.height,
                300,
                300,
                7);
    
    create_screen(screen.size.width, screen.size.height);
}

- (void)viewDidLayoutSubviews {
    [super viewDidLayoutSubviews];
    set_screen_size(self.view.frame.size.width, self.view.frame.size.height);
}

- (void)update {
    update_screen();
}

- (void)setup {
    
    self.preferredFramesPerSecond = 60;
    
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
