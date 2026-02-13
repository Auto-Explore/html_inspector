# html/semantics/scripting-1/the-script-element/microtasks/checkpoint-after-window-onerror-module.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/microtasks/checkpoint-after-window-onerror-module.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <meta charset="utf-8">
  <title>Microtask checkpoint after window.onerror events (module)</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="resources/checkpoint-after-error-event.js" type="module"></script>
  <script type="module">self.postMessage("foo");</script>
</head>
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
  "source_name": "html/semantics/scripting-1/the-script-element/microtasks/checkpoint-after-window-onerror-module.html"
}
```
