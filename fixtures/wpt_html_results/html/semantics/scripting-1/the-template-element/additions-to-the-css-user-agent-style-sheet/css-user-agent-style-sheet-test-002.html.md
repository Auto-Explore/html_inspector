# html/semantics/scripting-1/the-template-element/additions-to-the-css-user-agent-style-sheet/css-user-agent-style-sheet-test-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/additions-to-the-css-user-agent-style-sheet/css-user-agent-style-sheet-test-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
    <title>Template Test: check that template content is invisible by default</title>
    <link rel="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
    <link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#css-additions">
    <meta name="assert" content="The template element itself must be hidden by default">
    <link rel="match" href="css-user-agent-style-sheet-test-001-ref.html">
<body>
    <p>Test passes if there's no anything below this line.</p>
    <template style="border: 1px solid; width: 100px; height: 100px">
        <span style="color:red">Test fails if you can see this text or border around it</span>
    </template>
</body>
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
  "source_name": "html/semantics/scripting-1/the-template-element/additions-to-the-css-user-agent-style-sheet/css-user-agent-style-sheet-test-002.html"
}
```
