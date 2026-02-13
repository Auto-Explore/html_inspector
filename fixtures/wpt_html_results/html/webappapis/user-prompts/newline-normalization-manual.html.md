# html/webappapis/user-prompts/newline-normalization-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/user-prompts/newline-normalization-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Newline normalization in simple dialogs</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#simple-dialogs">

<p>The dialogs should all contain text looking like:</p>

<pre>Line 1.1
Line 1.2
Line 1.3
Line 1.4

Line 2.1</pre>

<script>
"use strict";

for (const func of [alert, confirm, prompt]) {
  func('Line 1.1\nLine 1.2\rLine 1.3\r\nLine 1.4\n\rLine 2.1');
}
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
  "source_name": "html/webappapis/user-prompts/newline-normalization-manual.html"
}
```
