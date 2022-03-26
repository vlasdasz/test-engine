//
//  AppDelegate.m
//  TestEngine
//
//  Created by Vladas Zakrevskis on 02.04.2021.
//

#import <GLKit/GLKit.h>

#include <OpenGLES/ES3/gl.h>

#import "AppDelegate.h"
#import "TestEngineController.h"

@interface AppDelegate()

@end

@implementation AppDelegate
- (void)applicationDidFinishLaunching:(UIApplication*)application {
    _window = [UIWindow new];
    _window.rootViewController = [TestEngineController new];
    [_window makeKeyAndVisible];
}
@end
