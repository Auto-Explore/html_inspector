# html/browsers/history/the-location-interface/location_port.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location_port.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>location_port</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    test(function () {
      var port = location.port;
      var url = location.href;

      var pos = url.indexOf("//");
      if (pos != -1) {
        url = url.substr(pos+2, url.length-pos-2);
        pos = url.indexOf("/");
        if (pos != -1)
          url = url.substr(0, pos);
        pos = url.indexOf(":");
        if (pos != -1)
          url = url.substr(pos+1, url.length-pos-1);
      }

      assert_equals(port, url, "port");

    }, "location port");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/history/the-location-interface/location_port.html"
}
```
