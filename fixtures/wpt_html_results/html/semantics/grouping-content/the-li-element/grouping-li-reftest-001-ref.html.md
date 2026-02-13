# html/semantics/grouping-content/the-li-element/grouping-li-reftest-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>li element</title>
    <link rel="author" title="dzenana" href="mailto:dzenana.trenutak@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-li-element">
    <meta name="assert" content="The value attribute has no effect when applied to a li element whose parent is a non-ol element." />
</head>
<body>
    <h1>Description</h1>
    <p>This test continues to validate the li element.</p>

    <p>This reftest verifies that the value attribute has no effect when applied to a list item NOT having an ol parent and not marked as display: list-item.</p>
    <p>A reftest is necessary because the values of li elements as calculated by the user agent are NOT available programatically. Only explicitly-set values are then available programatically.</p>
    <p>This reftest passes if you see NO sequencing information on any of the items below.</p>

    <p>Unordered List</p>
    <ul>
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ul>

    <menu>
        <li>Menu Item</li>
        <li>Menu Item</li>
    </menu>

    <menu type="toolbar">
        <li>
            <menu label="ToolbarLabel">
                <li><a>Toolbar Menu Item</a></li>
                <li><a>Toolbar Menu Item</a></li>
            </menu>
        </li>
        <li>
            <menu label="ToolbarLabel">
                <li><a>Toolbar Menu Item</a></li>
                <li><a>Toolbar Menu Item</a></li>
            </menu>
        </li>
     </menu>

</body>
</html>
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-001-ref.html"
}
```
