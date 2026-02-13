# html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_crossorigin.sub.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_crossorigin.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  function draw_and_read_image(img, should_throw) {
      let c = document.createElement('canvas');
      document.body.appendChild(c);
      let ctx = c.getContext('2d');
      ctx.drawImage(img, 0, 0);

      function get_image_data() {
          ctx.getImageData(0, 0, 4, 4);
      }

      if (should_throw) {
          assert_throws_dom('SecurityError', get_image_data);
      } else {
          get_image_data();
      }

      document.body.removeChild(c);
  }

  async_test(t => {
      let img = new Image();
      img.src = "/images/green.png";
      img.crossOrigin = "anonymous";
      img.onload = t.step_func_done(() => {
          draw_and_read_image(img, false);
      });
      img.onerror = t.unreached_func();
  }, "Can get pixels of canvas with same origin image drawn");

  async_test(t => {
      let img = new Image();
      img.src = "http://{{hosts[][www]}}:{{ports[http][0]}}/images/green.png?pipe=header(Access-Control-Allow-Origin,*)";
      img.crossOrigin = "anonymous";
      img.onload = t.step_func_done(() => {
          draw_and_read_image(img, false);
      });
      img.onerror = t.unreached_func();
  }, "Can get pixels of canvas with CORS enabled cross origin image drawn");

  async_test(t => {
      let img = new Image();
      img.src = "http://{{hosts[][www]}}:{{ports[http][0]}}/images/green.png?pipe=header(Access-Control-Allow-Origin,*)";
      img.onload = t.step_func_done(() => {
          draw_and_read_image(img, true);
      });
      img.onerror = t.unreached_func();
  }, "Can't get pixels of canvas with CORS enabled cross origin image drawn from non-CORS element");

  async_test(t => {
      let img = new Image();
      img.src = "http://{{hosts[][www]}}:{{ports[http][0]}}/images/green.png";

      img.onload = t.step_func_done(() => {
          draw_and_read_image(img, true);
      });
      img.onerror = t.unreached_func();
  }, "Can't get pixels of canvas with non-CORS enabled cross origin image drawn");
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
        "byte_end": 40,
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
  "source_name": "html/canvas/element/manual/drawing-images-to-the-canvas/drawimage_crossorigin.sub.html"
}
```
