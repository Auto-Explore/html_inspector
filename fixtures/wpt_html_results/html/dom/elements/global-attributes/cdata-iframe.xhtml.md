# html/dom/elements/global-attributes/cdata-iframe.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/cdata-iframe.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml">
    <body>
        <div id="container"></div>
    </body>

    <script>
        function createXHTMLCase(id) {
            const container = document.getElementById("container");

            const div = document.createElement("div");
            div.dir = "auto";
            div.id = id;

            const cdata = document.createCDATASection("foo");
            const text = document.createTextNode("اختبر");
            div.appendChild(cdata);
            div.appendChild(text);

            container.appendChild(div);

            return [div, cdata];
        }

        window.addEventListener("message", (e) => {
            createXHTMLCase(e.data);
            window.top.postMessage(e.data);
        });

        window.top.postMessage("subframe-loaded");
    </script>
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
  "source_name": "html/dom/elements/global-attributes/cdata-iframe.xhtml"
}
```
