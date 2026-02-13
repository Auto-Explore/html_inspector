# html/semantics/popovers/popover-anchor-idl-property.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-idl-property.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div>
  <button id=b1>This is an anchor button</button>
  <div popover id=p1 anchor=b1>This is a popover</div>
  <button id=b2 popovertarget=p1>This button invokes the popover but isn't an anchor</button>
</div>

<script>
  test(function() {
    assert_equals(p1.anchorElement,b1);
  }, "popover anchorElement IDL property returns the anchor element");

  test(function() {
    assert_equals(p1.anchorElement,b1);
    p1.anchorElement = b2;
    assert_equals(p1.anchorElement,b2);
    assert_equals(p1.getAttribute('anchor'),'','Idref is empty after setting element');
    p1.anchorElement = b1; // Reset
  }, "popover anchorElement is settable");
</script>

<div>
  <button id=b3>button</button>
  <div id=p2>Anchored div</div>
</div>
<style>
  * {margin:0;padding:0;}
  #b3 {width: 200px;}
  #p2 {
    position: absolute;
    left: anchor(right);
  }
</style>

<script>
  test(function() {
    assert_equals(p2.anchorElement,null);
    const button = document.getElementById('b3');
    assert_true(!!button);
    p2.anchorElement = button;
    assert_equals(p2.getAttribute('anchor'),'','Idref should be empty after setting element');
    assert_equals(p2.anchorElement,button,'Element reference should be button');
    assert_equals(p2.offsetLeft, 200, 'The anchor relationship should be functional');
  }, "anchorElement affects anchor positioning");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1099,
        "byte_start": 1092,
        "col": 1,
        "line": 33
      }
    },
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
  "source_name": "html/semantics/popovers/popover-anchor-idl-property.tentative.html"
}
```
