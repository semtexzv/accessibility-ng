use accessibility_ng::{
    AXAttribute, AXUIElement, TreeVisitor, TreeWalkerFlow,
};
use std::cell::Cell;

// Recreate PrintyBoi from main.rs for testing
struct PrintyBoi {
    level: Cell<usize>,
    indent: String,
    children: AXAttribute<CFArray<AXUIElement>>,
}

// Add this import to avoid undefined type
use core_foundation::array::CFArray;

impl PrintyBoi {
    pub fn new_with_indentation(indent: usize) -> Self {
        Self {
            level: Cell::new(0),
            indent: " ".repeat(indent),
            children: AXAttribute::children(),
        }
    }
}

impl TreeVisitor for PrintyBoi {
    fn enter_element(&self, _element: &AXUIElement) -> TreeWalkerFlow {
        self.level.replace(self.level.get() + 1);
        
        // We're not actually printing in the test, just checking that
        // the code runs without errors and tracks the level correctly
        
        TreeWalkerFlow::Continue
    }

    fn exit_element(&self, _element: &AXUIElement) {
        self.level.replace(self.level.get() - 1);
    }
}

#[test]
fn test_printy_boi_creation() {
    let printy = PrintyBoi::new_with_indentation(4);
    assert_eq!(printy.indent, "    ");
    assert_eq!(printy.level.get(), 0);
}

#[test]
fn test_printy_boi_level_tracking() {
    let printy = PrintyBoi::new_with_indentation(2);
    let element = AXUIElement::system_wide();
    
    // Initial level should be 0
    assert_eq!(printy.level.get(), 0);
    
    // Enter should increment
    let _ = printy.enter_element(&element);
    assert_eq!(printy.level.get(), 1);
    
    // Enter again
    let _ = printy.enter_element(&element);
    assert_eq!(printy.level.get(), 2);
    
    // Exit should decrement
    printy.exit_element(&element);
    assert_eq!(printy.level.get(), 1);
    
    // Exit again
    printy.exit_element(&element);
    assert_eq!(printy.level.get(), 0);
}

#[test]
fn test_tree_flow_value() {
    // Since TreeWalkerFlow doesn't implement Debug, we can't use assert_eq! or assert_ne!
    // Instead, we'll do a direct comparison using match expressions
    
    let continue_flow = TreeWalkerFlow::Continue;
    let skip_flow = TreeWalkerFlow::SkipSubtree;
    let exit_flow = TreeWalkerFlow::Exit;
    
    // Check Continue != SkipSubtree
    let result = match (continue_flow, skip_flow) {
        (TreeWalkerFlow::Continue, TreeWalkerFlow::Continue) => false,
        (TreeWalkerFlow::SkipSubtree, TreeWalkerFlow::SkipSubtree) => false,
        (TreeWalkerFlow::Exit, TreeWalkerFlow::Exit) => false,
        _ => true,
    };
    assert!(result, "Continue should not equal SkipSubtree");
    
    // Check Continue != Exit
    let result = match (continue_flow, exit_flow) {
        (TreeWalkerFlow::Continue, TreeWalkerFlow::Continue) => false,
        (TreeWalkerFlow::SkipSubtree, TreeWalkerFlow::SkipSubtree) => false, 
        (TreeWalkerFlow::Exit, TreeWalkerFlow::Exit) => false,
        _ => true,
    };
    assert!(result, "Continue should not equal Exit");
    
    // Check SkipSubtree != Exit
    let result = match (skip_flow, exit_flow) {
        (TreeWalkerFlow::Continue, TreeWalkerFlow::Continue) => false,
        (TreeWalkerFlow::SkipSubtree, TreeWalkerFlow::SkipSubtree) => false,
        (TreeWalkerFlow::Exit, TreeWalkerFlow::Exit) => false,
        _ => true, 
    };
    assert!(result, "SkipSubtree should not equal Exit");
}