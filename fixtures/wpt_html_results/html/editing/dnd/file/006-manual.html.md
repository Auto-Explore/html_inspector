# html/editing/dnd/file/006-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/file/006-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - cancelling dragging text onto a file input</title>

<ol>
  <li>Save <a href="../resources/filler.html">this file</a> to your computer.</li>
  <li>Write the full /path/and/name to that file, into the first input below, then select all of the text you just entered.</li>
  <li>Drag selected text to the file input. If no prompt appears, and the text is not added to the file input, pass and ignore further steps.</li>
  <li>If a prompt appears, refuse it. Fail if the file input's value is set without any prompts.</li>
  <li>If a prompt appears; fail if the file input's value is set after refusing the prompt.</li>
</ol>
<p><input value="/tmp/filler.html"></p>
<p><input type="file"></p>
<noscript><p>Enable JavaScript and reload</p></noscript>
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
  "source_name": "html/editing/dnd/file/006-manual.html"
}
```
