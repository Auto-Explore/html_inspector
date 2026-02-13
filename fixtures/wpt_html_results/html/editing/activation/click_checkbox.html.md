# html/editing/activation/click_checkbox.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/activation/click_checkbox.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Interaction of UI input and the click in progress flag</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<p>When you mouse click the checkbox below it should not be checked:</p>
<p><input type=checkbox onclick=this.click() id="target"></p>
<p>Now keyboard "click" the checkbox and confirm it's still not checked.</p>
<script>
promise_test(async t => {
  var target = document.getElementById("target");
  var received = false;
  target.addEventListener("click", t.step_func(function(e) {
  	received = true;
    assert_false(target.checked, "The checkbox should not be checked")
  }));

  await test_driver.click(target);
  assert_true(received, "click event should have been dispatched synchronously");
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
  "source_name": "html/editing/activation/click_checkbox.html"
}
```
