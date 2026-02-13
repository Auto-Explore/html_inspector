# html/editing/activation/click.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/activation/click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLElement#click</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function() {
  var element = document.createElement("div");
  var received = false;
  element.addEventListener("click", this.step_func(function(e) {
    received = true;
    assert_false(e.isTrusted, "Event should not be trusted")
  }));
  element.click();
  assert_true(received, "click event should have been dispatched synchronously");
})
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
  "source_name": "html/editing/activation/click.html"
}
```
