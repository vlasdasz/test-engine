//
//  AppDelegate.m
//  TestEngine
//
//  Created by Vladas Zakrevskis on 02.04.2021.
//

#import <GLKit/GLKit.h>

#import "AppDelegate.h"

@interface Controller : GLKViewController

@end

@implementation Controller

- (void)viewDidLoad {
    [super viewDidLoad];
    [self setup];

}

- (void)update {

}

- (void)setup {
    
    self.preferredFramesPerSecond = 60;
    
    EAGLContext* context = [[EAGLContext alloc] initWithAPI:kEAGLRenderingAPIOpenGLES3];
    
    if (context == nil) {
        context = [[EAGLContext alloc] initWithAPI:kEAGLRenderingAPIOpenGLES2];
        NSLog(@"%s", "kEAGLRenderingAPIOpenGLES2");
    }
    else {
        NSLog(@"%s", "kEAGLRenderingAPIOpenGLES3");
    }

    NSLog(@"%@", context);
    [EAGLContext setCurrentContext:context];
    GLKView* view = (GLKView*)self.view;
    view .context = context;
    view.drawableColorFormat = GLKViewDrawableColorFormatRGBA8888;
    view.drawableDepthFormat = GLKViewDrawableDepthFormat16;
    view.drawableStencilFormat = GLKViewDrawableStencilFormat8;
    view.multipleTouchEnabled = true;
}

- (void)didRotateFromInterfaceOrientation:(UIInterfaceOrientation)fromInterfaceOrientation {

}

- (void)touchesBegan:(NSSet<UITouch*>*)touches withEvent:(UIEvent*)event {

}

- (void)touchesMoved:(NSSet<UITouch*>*)touches withEvent:(UIEvent*)event {

}

- (void)touchesEnded:(NSSet<UITouch*>*)touches withEvent:(UIEvent*)event {
 
}

@end

@interface AppDelegate()

@end

@implementation AppDelegate
- (void)applicationDidFinishLaunching:(UIApplication*)application {
    _window = [UIWindow new];
    _window.rootViewController = [Controller new];
    [_window makeKeyAndVisible];
}
@end
