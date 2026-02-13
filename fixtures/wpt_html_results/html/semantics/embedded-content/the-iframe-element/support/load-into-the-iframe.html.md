# html/semantics/embedded-content/the-iframe-element/support/load-into-the-iframe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/load-into-the-iframe.html",
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
  </head>
  <body>
    <iframe sandbox="allow-scripts"></iframe>
    <script>
      let frame = document.querySelector("iframe");
      let sandbox = new URL(location.href).searchParams.get("sandbox");
      if (sandbox) {
        frame.sandbox = sandbox;
      }
      // We're the popup (i.e. a top frame).  Load into the iframe the page
      // trying to modifying the top frame and transmit the result to our
      // opener.
      onmessage = function(e) {
        opener.postMessage(e.data, "*")
      }
      frame.src = "iframe-that-performs-top-navigation-on-popup.html";
    </script>
  </body>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/load-into-the-iframe.html"
}
```
