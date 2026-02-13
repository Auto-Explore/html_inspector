# html/browsers/windows/embedded-opener.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/embedded-opener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>opener and embedded documents; using window.open()</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<iframe name=matchesastring></iframe>
<script>
async_test(t => {
  const frame = document.querySelector("iframe");
  frame.onload = t.step_func(() => {
    // Firefox and Chrome/Safari load differently
    if (frame.contentWindow.location.href === "about:blank") {
      return;
    }

    // Test bits
    assert_equals(frame.contentWindow.opener, window, "opener before setting it to null");

    const openerDesc = Object.getOwnPropertyDescriptor(frame.contentWindow, "opener"),
          openerGet = openerDesc.get;

    assert_equals(openerGet(), window, "opener before setting it to null via directly invoking the getter");
    frame.contentWindow.opener = null;
    frame.contentWindow.opener = "immaterial";
    assert_equals(openerGet(), null, "opener after setting it to null via directly invoking the getter");
    assert_equals(frame.contentWindow.opener, "immaterial");

    t.done();
  });
  window.open("/common/blank.html", "matchesastring");
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
  "source_name": "html/browsers/windows/embedded-opener.html"
}
```
