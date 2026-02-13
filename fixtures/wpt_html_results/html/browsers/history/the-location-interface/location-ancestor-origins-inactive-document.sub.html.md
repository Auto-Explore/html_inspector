# html/browsers/history/the-location-interface/location-ancestor-origins-inactive-document.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-ancestor-origins-inactive-document.sub.html",
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
    <title>Location.ancestorOrigins lifetime behavior</title>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/browsing-the-web.html#dom-location-ancestororigins">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      function createIframeAndNavigate(test, src) {
        return new Promise(resolve => {
          const iframe = document.createElement("iframe");
          iframe.onload = () => {
            resolve(iframe);
          }
          iframe.src = src;
          document.body.appendChild(iframe);
          test.add_cleanup(() => {
            iframe.remove();
          });
        });
      }


      promise_test(async t => {
        assert_implements(location.ancestorOrigins, "location.ancestorOrigins not implemented");
        const iframe = await createIframeAndNavigate(t, "about:blank");
        assert_array_equals(
          iframe.contentWindow.location.ancestorOrigins,
          [location.origin],
          "Initial ancestorOrigins should match expected placeholder value"
        );

        const loc = iframe.contentWindow.location;
        iframe.remove();

        assert_array_equals(
          Array.from(loc.ancestorOrigins),
          [],
          "ancestorOrigins should be empty after iframe removal"
        );
      }, "location.ancestorOrigins returns empty list after iframe is removed and referenced Location's relevant document is null");

      promise_test(async t => {
        assert_implements(location.ancestorOrigins, "location.ancestorOrigins not implemented");
        const iframe = await createIframeAndNavigate(t, "about:blank");
        assert_array_equals(
          iframe.contentWindow.location.ancestorOrigins,
          [location.origin],
          "Initial ancestorOrigins should match expected placeholder value"
        );

        const loc = iframe.contentWindow.location;
        await new Promise(resolve => {
          iframe.onload = resolve;
          iframe.src = "http://{{hosts[alt][]}}:{{ports[http][0]}}/common/blank.html";
        });

        assert_array_equals(
          Array.from(loc.ancestorOrigins),
          [],
          "ancestorOrigins should be empty after iframe navigation"
        );
      }, "location.ancestorOrigins returns empty list when iframe navigated away and referenced Location's relevant document is null");
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
  "source_name": "html/browsers/history/the-location-interface/location-ancestor-origins-inactive-document.sub.html"
}
```
