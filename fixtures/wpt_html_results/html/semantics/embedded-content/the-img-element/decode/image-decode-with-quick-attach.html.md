# html/semantics/embedded-content/the-img-element/decode/image-decode-with-quick-attach.html

Counts:
- errors: 6
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/decode/image-decode-with-quick-attach.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<meta name="timeout" content="long">
<title>HTMLImageElement.prototype.decode(), attach to DOM before promise resolves.</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/embedded-content.html#dom-img-decode">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
  <picture id="empty-picture-1"></picture>
  <picture id="empty-picture-2"></picture>
  <picture id="empty-picture-3"></picture>
  <picture id="picture-with-source-1">
    <source srcset="/images/blue.png"></source>
  </picture>
  <picture id="picture-with-source-2">
    <source srcset="/images/blue.png"></source>
  </picture>
  <picture id="picture-with-source-3">
    <source srcset="/images/blue.png"></source>
  </picture>
</body>

<script>
"use strict";

let png = "/images/green.png?image-decode-with-quick-attach-" + Math.random();

function run_test(t, {
  image = png,
  prop = "src",
  container = document.body,
  expectReject = false
} = {}) {
  const img = new Image();
  img[prop] = image;
  const promise = img.decode().then(function (arg) {
    assert_equals(arg, undefined);
  });
  // Intentionally don't wait for the promise to settle before attaching the image.
  container.appendChild(img);
  return expectReject ?
    promise_rejects_dom(t, "EncodingError", promise) :
    promise;
}

promise_test(t => run_test(t), document.title + ": src not cached");
promise_test(t => run_test(t), document.title + ": src cached");
promise_test(t => run_test(t, {prop: "srcset"}), document.title + ": srcset");
promise_test(t => run_test(t, {
  image: png + "-picture",
  container: document.getElementById("empty-picture-1"),
  expectReject: true,
}), document.title + ": src in empty picture not cached");
promise_test(t => run_test(t, {
  image: png + "-picture",
  container: document.getElementById("empty-picture-2"),
  // NOTE(emilio): This is inconsistent between the cached and un-cached case
  // below, but actually correct per spec, because in the sync case the current
  // request doesn't mutate after the decode() call.
  // This is expected to change in https://github.com/whatwg/html/issues/10531
  expectReject: false,
}), document.title + ": src in empty picture cached");
promise_test(t => run_test(t, {
  image: png + "-picture-srcset",
  prop: "srcset",
  container: document.getElementById("empty-picture-3"),
  expectReject: true,
}), document.title + ": srcset in empty picture");
promise_test(t => run_test(t, {
  image: png + "-picture-with-source",
  container: document.getElementById("picture-with-source-1"),
  expectReject: true,
}), document.title + ": src in picture with source not cached");
promise_test(t => run_test(t, {
  image: png + "-picture-with-source",
  container: document.getElementById("picture-with-source-2"),
  expectReject: true,
}), document.title + ": src in picture with source cached");
promise_test(t => run_test(t, {
  image: png + "-picture-with-source-srcset",
  prop: "srcset",
  container: document.getElementById("picture-with-source-3"),
  expectReject: true,
}), document.title + ": srcset in picture with source");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 497,
        "byte_start": 487,
        "col": 33,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 540,
        "byte_start": 530,
        "col": 33,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 583,
        "byte_start": 573,
        "col": 33,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “source”.",
      "severity": "Error",
      "span": {
        "byte_end": 670,
        "byte_start": 661,
        "col": 39,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 683,
        "byte_start": 673,
        "col": 3,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “source”.",
      "severity": "Error",
      "span": {
        "byte_end": 770,
        "byte_start": 761,
        "col": 39,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 783,
        "byte_start": 773,
        "col": 3,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “source”.",
      "severity": "Error",
      "span": {
        "byte_end": 870,
        "byte_start": 861,
        "col": 39,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 883,
        "byte_start": 873,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 901,
        "byte_start": 893,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 3248,
        "byte_start": 901,
        "col": 9,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 3257,
        "byte_start": 3248,
        "col": 1,
        "line": 87
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
  "source_name": "html/semantics/embedded-content/the-img-element/decode/image-decode-with-quick-attach.html"
}
```
