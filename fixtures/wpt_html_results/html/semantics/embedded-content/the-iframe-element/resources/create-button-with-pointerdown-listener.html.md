# html/semantics/embedded-content/the-iframe-element/resources/create-button-with-pointerdown-listener.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/resources/create-button-with-pointerdown-listener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- This document is used for the test `move-node-local-root-events-still-fire.html`.
     This document is a grandchild of that document, and is directly hosted by the
     intermediate `cross-origin-middle-frame-2.html` -->

<button id=button>Grandchild button</button>
<script>
  window.events = [];
  button.addEventListener('pointerdown', e => {
    window.events.push('pointerdown');
  });
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/resources/create-button-with-pointerdown-listener.html"
}
```
