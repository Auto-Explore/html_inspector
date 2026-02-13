# html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focus - the sequential focus navigation order</title>
<meta name="timeout" content="long">
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#sequential-focus-navigation">
<meta assert="flag" content="interact">
<meta assert="assert" content="Check the sequential focus navigation order">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<div id="log"></div>
<form id="fm">
  <button id="btn0">tabindex(omitted)</button>
  <button id="btn1" tabindex="">tabindex(empty)</button>
  <button id="btn2" tabindex="a">tabindex(a)</button>
  <button id="btn3" tabindex="-1">tabindex(-1)</button>
  <button id="btn4" tabindex="0">tabindex(0)</button>
  <button id="btn5" tabindex="3">tabindex(3)</button>
  <button id="btn6" tabindex="2">tabindex(2)</button>
  <button id="btn7" tabindex="2">tabindex(2)</button>
  <button id="btn8" tabindex="2">tabindex(2)</button>
  <button id="btn9" tabindex="1">tabindex(1)</button>
</form>
<script>

var i = 0,
    expectation = ["btn9", "btn6", "btn7", "btn8", "btn5", "btn0", "btn1", "btn2", "btn4"],
    results = [],
    t = async_test("Elements with different tabindex must be focused sequentially when pressing 'Tab' keys");

setup(function () {
  document.body.focus();
});



document.forms.fm.addEventListener("focus", function (evt) {
  results.push(evt.target.id);
  if (i >= 8) {
    t.step(function () {
      assert_array_equals(results, expectation);
    });
    t.done();
  } else {
    t.step(function () {
      // TAB = '\ue004'
      test_driver.send_keys(document.body, "\ue004");
    });
  }
  i++;
}, true);

document.addEventListener("keydown", function (evt) {
  t.step(function () {
    assert_equals(evt.keyCode, 9, "Please press 'Tab' key.");
  });
}, true);

t.step(function () {
  // TAB = '\ue004'
  test_driver.send_keys(document.body, "\ue004");
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-order.html"
}
```
