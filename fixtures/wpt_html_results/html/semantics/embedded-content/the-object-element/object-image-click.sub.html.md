# html/semantics/embedded-content/the-object-element/object-image-click.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-image-click.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTML Test: object - attributes</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<object id="same-origin-explicit" data="/images/green.png" type="image/png"></object>
<object id="same-origin-implicit" data="/images/green.png"></object>
<object id="cross-origin-explicit" data="http://{{hosts[][www]}}:{{ports[http][0]}}/images/green.png" type="image/png"></object>
<object id="cross-origin-implicit" data="http://{{hosts[][www]}}:{{ports[http][0]}}/images/green.png"></object>
<script>
function test_click(object) {
  promise_test(async function() {
    if (document.readyState != "complete") {
      await new Promise(r => window.addEventListener("load", r, { once: true }));
    }
    let p = new Promise(resolve => {
      object.addEventListener("click", function(e) {
        assert_true(true, `${object.id} was clicked`);
        resolve();
      }, { once: true });
    });
    await test_driver.click(object);
    await p;
  }, object.id);
}

for (let origin of ["same-origin", "cross-origin"]) {
  for (let implicit of ["explicit", "implicit"]) {
    let id = origin + "-" + implicit;
    let object = document.getElementById(id);
    test_click(object);
  }
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.invalid",
      "message": "Bad value “http://{{hosts[][www]}}:{{ports[http][0]}}/images/green.png” for attribute “data” on element “object”.",
      "severity": "Warning",
      "span": {
        "byte_end": 771,
        "byte_start": 652,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.invalid",
      "message": "Bad value “http://{{hosts[][www]}}:{{ports[http][0]}}/images/green.png” for attribute “data” on element “object”.",
      "severity": "Warning",
      "span": {
        "byte_end": 883,
        "byte_start": 781,
        "col": 1,
        "line": 14
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-image-click.sub.html"
}
```
