# html/browsers/browsing-the-web/history-traversal/support/window-name-navigation.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/support/window-name-navigation.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <script>
        var url = new URL(window.location.href);
        url.hostname = "{{GET[hostname]}}";
        url.pathname = "/html/browsers/browsing-the-web/history-traversal/support/window-name-test.sub.html";
        url.search = "shouldhavename={{GET[shouldhavename]}}&sendmessage={{GET[sendmessage]}}";
        window.name = "test";
        window.location = url.href;
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/support/window-name-navigation.sub.html"
}
```
