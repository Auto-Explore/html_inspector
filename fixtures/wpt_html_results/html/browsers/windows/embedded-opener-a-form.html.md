# html/browsers/windows/embedded-opener-a-form.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/embedded-opener-a-form.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>opener and embedded documents; using a and form</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<iframe name=matchesastring></iframe>
<a href=/common/blank.html target=matchesastring>&lt;a></a>
<form action=/common/blank.html target=matchesastring><input type=submit value="<form>"></form>
<script>
async_test(t => {
  const frame = document.querySelector("iframe");
  let counter = 0;
  frame.onload = t.step_func(() => {
    // Firefox and Chrome/Safari load differently
    if (frame.contentWindow.location.href === "about:blank") {
      return;
    }

    // Test bits
    assert_equals(frame.contentWindow.opener, null);
    if (counter === 0) {
      document.querySelector("input").click();
    } else {
      t.done();
    }
    counter++;
  });
  document.querySelector("a").click();
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
  "source_name": "html/browsers/windows/embedded-opener-a-form.html"
}
```
