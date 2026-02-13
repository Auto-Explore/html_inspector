# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_helper-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_helper-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script>
  if (opener) {
    // We're the popup.  Send back our state.  What we really want to send is
    // our origin, but that will come automatically.
    opener.postMessage(undefined, "*");
    self.close();
  } else {
    // We're the child.  Start listening for messages and open ourselves as the
    // popup.
    onmessage = function (e) {
      parent.postMessage({ data: e.data, origin: e.origin }, "*");
    };
    popupWin = window.open(location.href);
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_helper-1.html"
}
```
