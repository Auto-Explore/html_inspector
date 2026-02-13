# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_helper-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_helper-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
<script>
  if (opener) {
    // We're the popup.  Send back our state.  What we really want to send is
    // our origin, but that will come automatically.
    opener.postMessage(undefined, "*");
    self.close();
  } else {
    // We're the child.  Start listening for messages from our parent and open
    // ourselves as the popup when we get the "start" message.
    onmessage = function (e) {
      if (e.data == "start") {
        // Now listen for messages from the thing we plan to open.
        onmessage = function(e) {
          parent.postMessage({ data: e.data, origin: e.origin }, "*");
        }

        var a = document.createElement("a");
        a.href = location.href;
        a.target = "_blank";
        a.rel = "opener";
        document.body.appendChild(a);
        a.click();
      }
    };
  }
</script>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_helper-2.html"
}
```
