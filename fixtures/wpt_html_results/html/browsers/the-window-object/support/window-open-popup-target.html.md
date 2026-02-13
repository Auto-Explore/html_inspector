# html/browsers/the-window-object/support/window-open-popup-target.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/support/window-open-popup-target.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script>
  var channelName = window.name;
  var channel = new BroadcastChannel(channelName);
  const allBarProps = [
    window.locationbar.visible,
    window.menubar.visible,
    window.personalbar.visible,
    window.scrollbars.visible,
    window.statusbar.visible,
    window.toolbar.visible
  ];
  const allTrue = allBarProps.every(x=>x);
  const allFalse = allBarProps.every(x=>!x);
  channel.postMessage({isPopup: allFalse, mixedState: !allTrue && !allFalse});

  // Because messages are not delivered synchronously and because closing a
  // browsing context prompts the eventual clearing of all task sources, this
  // document should not be closed until the opener document has confirmed
  // receipt.
  channel.onmessage = function() {
    window.close();
  };
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
  "source_name": "html/browsers/the-window-object/support/window-open-popup-target.html"
}
```
