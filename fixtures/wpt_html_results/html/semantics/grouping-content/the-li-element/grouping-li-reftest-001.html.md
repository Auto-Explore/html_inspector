# html/semantics/grouping-content/the-li-element/grouping-li-reftest-001.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-001.html",
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
    <link rel="match" href="grouping-li-reftest-001-ref.html" />
    <meta name="assert" content="The value attribute has no effect when applied to a li element whose parent is a non-ol element." />
</head>
<body>
    <h1>Description</h1>
    <p>This test continues to validate the li element.</p>

    <p>This reftest verifies that the value attribute has no effect when applied to a list item NOT having an ol parent and not marked as display: list-item.</p>
    <p>A reftest is necessary because the values of li elements as calculated by the user agent are NOT available programatically.  Only explicitly-set values are then available programatically.</p>
    <p>This reftest passes if you see NO sequencing information on any of the items below.</p>

    <p>Unordered List</p>
    <ul>
        <li value="2">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ul>

    <menu>
        <li>Menu Item</li>
        <li value="3">Menu Item</li>
    </menu>

    <menu type="toolbar">
        <li value="4">
            <menu label="ToolbarLabel">
                <li value="5"><a>Toolbar Menu Item</a></li>
                <li><a>Toolbar Menu Item</a></li>
            </menu>
        </li>
        <li value="6">
            <menu label="ToolbarLabel">
                <li value="10"><a>Toolbar Menu Item</a></li>
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
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 1065,
        "byte_start": 1051,
        "col": 9,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 1205,
        "byte_start": 1191,
        "col": 9,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 1281,
        "byte_start": 1267,
        "col": 9,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 1352,
        "byte_start": 1338,
        "col": 17,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 1488,
        "byte_start": 1474,
        "col": 9,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 1560,
        "byte_start": 1545,
        "col": 17,
        "line": 40
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-001.html"
}
```
