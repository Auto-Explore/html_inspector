# html/syntax/charset/inheritance-bogus-meta.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/inheritance-bogus-meta.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=windows-1253>
<title>Inheriting from windows-1253 parent</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/get-host-info.sub.js></script>
<div id=log></div>
<script>
[
  {
    "title": "Child with bogus <meta charset>",
    "url": "resources/bogus-charset.html",
    "expected": "�\n" // 0x00A2 in windows-1252 is at the same position as 0x0386 in windows-1253
  },
  {
    "title": "Child with bogus Content-Type charset",
    "url": "resources/bogus-charset-http.py",
    "expected": "�\n"
  },
  {
    "title": "Child with bogus Content-Type charset, but valid <meta charset>",
    "url": "resources/bogus-charset-http-valid-meta.py",
    "expected": "\u045E\n"
  }
].forEach(({ title, url, expected }) => {
  async_test(t => {
    const frame = document.createElement("iframe");
    t.add_cleanup(() => frame.remove());
    frame.src = url;
    frame.onload = t.step_func_done(() => {
      assert_equals(frame.contentDocument.body.textContent, expected);
    });
    document.body.append(frame);
  }, title);
});

async_test(t => {
  self.onmessage = t.step_func_done(({ data }) => {
    assert_equals(data, "\u00A2\n");
  });
  const frame = document.createElement("iframe");
  t.add_cleanup(() => frame.remove());
  frame.src = get_host_info().HTTP_REMOTE_ORIGIN + new URL("resources/bogus-charset.html", location).pathname + "?postMessage";
  document.body.append(frame);
}, "Cross-origin child with bogus <meta charset>");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1253” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 43,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/charset/inheritance-bogus-meta.html"
}
```
