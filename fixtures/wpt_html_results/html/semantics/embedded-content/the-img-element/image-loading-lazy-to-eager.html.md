# html/semantics/embedded-content/the-img-element/image-loading-lazy-to-eager.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-loading-lazy-to-eager.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Below-viewport images with loading='lazy' load when set to
         loading='eager' or the `loading` attribute is removed</title>
  <link rel="author" title="Dom Farolino" href="mailto:domfarolino@gmail.com">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/embedded-content.html#attr-img-loading">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>

<script>
  const t = async_test("Below-viewport images with loading='lazy' load when " +
                       "set to loading='eager' or the `loading` attribute is " +
                       "removed");

  const img_1_onload = t.unreached_func("#img_1 should not load before the " +
                                        "window load event");
  const img_2_onload = t.unreached_func("#img_2 should not load before the " +
                                        "window load event");

  window.addEventListener("load", t.step_func(() => {
    const img_1 = document.querySelector('#img_1');
    const img_2 = document.querySelector('#img_2');

    const img_1_promise = new Promise((resolve, reject) => {
      img_1.onerror = reject;
      img_1.onload = resolve;
    });

    const img_2_promise = new Promise((resolve, reject) => {
      img_2.onerror = reject;
      img_2.onload = resolve;
    });

    Promise.all([img_1_promise, img_2_promise])
      .then(t.step_func_done())
      .catch(t.unreached_func("The images should load successfully"));

    // Kick off the requests.
    img_1.loading = 'eager';
    img_2.removeAttribute('loading'); // unset the attribute, putting it in
                                      // the default (eager) state.
  }));

</script>

<body>
  <div style="height:1000vh;"></div>
  <img id="img_1"
       src="resources/image.png?lazy-to-eager-1"
       loading="lazy" onload="img_1_onload();">
  <img id="img_2"
       src="resources/image.png?lazy-to-eager-2"
       loading="lazy" onload="img_2_onload();">
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1905,
        "byte_start": 1793,
        "col": 3,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2020,
        "byte_start": 1908,
        "col": 3,
        "line": 52
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-loading-lazy-to-eager.html"
}
```
