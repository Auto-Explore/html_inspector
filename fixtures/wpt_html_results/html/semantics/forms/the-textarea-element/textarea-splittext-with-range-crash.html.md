# html/semantics/forms/the-textarea-element/textarea-splittext-with-range-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-splittext-with-range-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<script>
addEventListener("load", () => {
  const textarea = document.querySelector("textarea");
  const ul = document.createElement('ul');

  const textNodeInTextarea = document.createTextNode("");
  textarea.appendChild(textNodeInTextarea);
  document.documentElement.getBoundingClientRect();

  textarea.appendChild(ul);
  const range = document.createRange();
  range.selectNode(ul);

  textNodeInTextarea.data = "ab";
  textNodeInTextarea.splitText(1);
});
</script>
</head>
<body><textarea></textarea></body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-textarea-element/textarea-splittext-with-range-crash.html"
}
```
