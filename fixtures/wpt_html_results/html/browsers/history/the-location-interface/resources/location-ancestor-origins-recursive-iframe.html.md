# html/browsers/history/the-location-interface/resources/location-ancestor-origins-recursive-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/resources/location-ancestor-origins-recursive-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Recursive iframe document used for Location.ancestorOrigins tests</title>
<!--- This file is used for the creation of the hosted pages inside the iframes. --->

<body>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <script src="./location-ancestor-origins-create-iframe.js"></script>
  <script>
    let iframe = null;

    window.addEventListener("message", async (e) => {
      // Message is not for us, try to pass it on...
      if (e.data.name !== window.name) {
        iframe?.contentWindow.postMessage(e.data, "*");
        return;
      }
      switch (e.data.action) {
        case Actions.addFrame: {
          iframe = document.createElement("iframe");
          await configAndNavigateIFrame(iframe, e.data.request);
          break;
        }
        case Actions.changeReferrerPolicy: {
          iframe.referrerPolicy = e.data.request.value;
          break;
        }
        case Actions.insertMetaElement: {
          const meta = document.createElement("meta");
          meta.name = "referrer";
          meta.content = e.data.request.value;
          document.head.appendChild(meta);
          break;
        }
        default:
          window.top.postMessage(e.data, "*");
      }
    });

    window.top.postMessage({ event: "sentOrigins", name: window.name, origins: Array.from(location.ancestorOrigins) }, "*");
  </script>
</body>
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
  "source_name": "html/browsers/history/the-location-interface/resources/location-ancestor-origins-recursive-iframe.html"
}
```
