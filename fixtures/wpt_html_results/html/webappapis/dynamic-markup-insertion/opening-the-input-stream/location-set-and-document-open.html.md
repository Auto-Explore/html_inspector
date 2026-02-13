# html/webappapis/dynamic-markup-insertion/opening-the-input-stream/location-set-and-document-open.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/location-set-and-document-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title></title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<body>
  <script>
    var t = async_test("Location sets should cancel current navigation and prevent later document.open() from doing anything");

    var finishTest = t.step_func_done(function() {
        assert_equals(frames[0].document.body.textContent, "PASS",
                      "Should not have FAIL in our textContent");
    });

    t.step(function() {
        var i = document.createElement("iframe");
        i.srcdoc = `
          <script>
            var blob = new Blob(["PASS"], { type: "text/html" });
            var url = URL.createObjectURL(blob);
            location.href = url;
            frameElement.onload = parent.finishTest;
            document.open();
            document.write("FAIL");
            document.close();
          <\/script>`;
        document.body.appendChild(i);
    });

  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 44,
        "byte_start": 37,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/location-set-and-document-open.html"
}
```
