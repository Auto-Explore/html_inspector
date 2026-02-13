# html/dom/elements/images/bypass-cache-revalidation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/images/bypass-cache-revalidation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cached images can bypass revalidation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<div id="imageDiv1"></div>
<div id="imageDiv2"></div>
<canvas id="canvas"></canvas>
<script>

function getImagePixel(image)
{
    canvas.getContext("2d").drawImage(image, 0, 0, 10, 10);
    return canvas.getContext("2d").getImageData(0, 0, 1, 1).data;
}

let resolve;
promise_test(async (t) => {
   const url = "image.py?id=" + token();

   let promise = new Promise(r => resolve = r);
   imageDiv1.innerHTML = `<img src="${url}" onload="resolve()"></img>`;
   await promise;

   const url2 = "image.py?id=" + token();
   promise = new Promise(r => resolve = r);
   imageDiv1.innerHTML = `<img src="${url2}" onload="resolve()"></img>`;
   await promise;

   promise = new Promise(r => resolve = r);
   imageDiv2.innerHTML = `<img id="image2" src="${url}" onload="resolve()"></img>`;
   await promise;

   assert_array_equals(getImagePixel(image2), [0, 255, 0, 255]);
}, "Images can bypass no-cache");
</script>

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
  "source_name": "html/dom/elements/images/bypass-cache-revalidation.html"
}
```
