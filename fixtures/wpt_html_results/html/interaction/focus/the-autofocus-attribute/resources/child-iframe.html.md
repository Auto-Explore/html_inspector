# html/interaction/focus/the-autofocus-attribute/resources/child-iframe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/resources/child-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/common/get-host-info.sub.js"></script>
<iframe id="iframe" width="200" height="100"></iframe>

<script>
  iframe.src =
    get_host_info().ORIGIN + "/html/interaction/focus/the-autofocus-attribute/resources/grand-child-autofocus.html";
  window.addEventListener("message", event => {
    if (event.data == "grand_child_loaded") {
      parent.postMessage("ready", "*");
    } else if (event.data == "report_focus_state") {
      frames[0].postMessage("report_focus_state", "*");
    } else if (event.data == "grand_child_is_focused" ||
               event.data == "grand_child_is_not_focused") {
      parent.postMessage(event.data, "*");
    }
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/resources/child-iframe.html"
}
```
