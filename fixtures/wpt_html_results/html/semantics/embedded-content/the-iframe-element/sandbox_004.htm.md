# html/semantics/embedded-content/the-iframe-element/sandbox_004.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_004.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Sandbox: Block plugins inside iframe with sandbox attribute</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  https://github.com/whatwg/html/issues/3958
  https://github.com/whatwg/html/pull/6946
-->

<iframe sandbox="allow-same-origin" src="support/iframe_sandbox_004.htm" height="400" width ="600"></iframe>

<script>
"use strict";
setup({ explicit_done: true });

window.onload = () => {
  test(() => {
    const object = document.querySelector("iframe").contentWindow.document.querySelector("object");
    const rect = object.getBoundingClientRect();
    assert_less_than(rect.width, 300);
    assert_less_than(rect.height, 300);
  }, "Fallback content is always displayed for sandboxed PDFs");
  done();
};
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_004.htm"
}
```
