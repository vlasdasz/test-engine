//
//  main.m
//  TestEngine
//
//  Created by Vladas Zakrevskis on 02.04.2021.
//


#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>

int test_game(void);

int main(int argc, char * argv[]) {
    return test_game();
}

UITextField* text_field = nil;

void ios_init_text_field(void) {

}

void ios_show_keyboard(void) {
    assert(text_field == nil);
    UIViewController *controller = [UIApplication sharedApplication].keyWindow.rootViewController;
    text_field = [UITextField new];
    [text_field setFrame:CGRectMake(100, 100, 300, 100)];
    [controller.view addSubview:text_field];
    
    NSLog(@"UITextField initialized");
    
    
    NSLog(@"A?");
    NSLog(@"%@", [text_field description]);
    [text_field becomeFirstResponder];
}

void ios_hide_keyboard(void) {
    [text_field resignFirstResponder];
}
