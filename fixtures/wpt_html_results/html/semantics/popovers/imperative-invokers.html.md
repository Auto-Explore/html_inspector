# html/semantics/popovers/imperative-invokers.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/imperative-invokers.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:masonf@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/9144#issuecomment-2195095228">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=unrelated></div>
<div id=popover popover=auto>
  <div id=contained></div>
  popover 1
</div>
<div id=popover2 popover=auto style="top:50px">
  popover 2
</div>

<script>
function testOneCase(shouldBeIndependent,popover2Opener,msg) {
  test((t) => {
    assert_false(popover.matches(':popover-open'),'starting state');
    assert_false(popover2.matches(':popover-open'),'starting state');
    t.add_cleanup(() => {popover.hidePopover();popover2.hidePopover()});
    popover.showPopover();
    assert_true(popover.matches(':popover-open'));
    popover2Opener();
    assert_true(popover2.matches(':popover-open'),'opener should open popover2');
    if (shouldBeIndependent) {
      assert_false(popover.matches(':popover-open'),'popovers should not be related');
    } else {
      assert_true(popover.matches(':popover-open'),'popovers should be related to each other');
    }
  },msg);
}

testOneCase(true,() => popover2.showPopover(),'normal opening');
testOneCase(true,() => popover2.showPopover({source: unrelated}),'showPopover(unrelated)');
testOneCase(false,() => popover2.showPopover({source: popover}),'showPopover(popover)');
testOneCase(false,() => popover2.showPopover({source: contained}),'showPopover(contained)');

testOneCase(true,() => popover2.togglePopover(true),'togglePopover(true)');
testOneCase(true,() => popover2.togglePopover({force:true}),'togglePopover({force})');
testOneCase(true,() => popover2.togglePopover({source:unrelated}),'togglePopover(unrelated)');
testOneCase(false,() => popover2.togglePopover({source: popover}),'togglePopover(popover)');
testOneCase(false,() => popover2.togglePopover({force:true, source: popover}),'togglePopover({force, popover})');

test(() => {
  assert_false(popover.matches(':popover-open'));
  assert_throws_js(TypeError,() => popover2.showPopover({source: null}),'showPopover(null)');
  assert_throws_js(TypeError,() => popover2.togglePopover({source:null}),'togglePopover(null)');
  assert_false(popover.matches(':popover-open'));
},'null isn\'t a valid Element');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/imperative-invokers.html"
}
```
