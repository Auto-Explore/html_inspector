# html/browsers/the-window-object/support/windowFeature-values-target.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/support/windowFeature-values-target.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script>
  const channelName = location.search.substr(1),
        channel = new BroadcastChannel(channelName);

  const haveOpener = window.opener !== null;
  const haveReferrer = document.referrer !== null && document.referrer !== "";
  const allBarProps = [
    window.locationbar.visible,
    window.menubar.visible,
    window.personalbar.visible,
    window.scrollbars.visible,
    window.statusbar.visible,
    window.toolbar.visible
  ];
  const isPopup = allBarProps.every(x=>!x);

  channel.postMessage({haveOpener, haveReferrer, isPopup});

  // Because messages are not delivered synchronously and because closing a
  // browsing context prompts the eventual clearing of all task sources, this
  // document should not be closed until the opener document has confirmed
  // receipt.
  channel.onmessage = () => window.close();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 8,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
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
  "source_name": "html/browsers/the-window-object/support/windowFeature-values-target.html"
}
```
