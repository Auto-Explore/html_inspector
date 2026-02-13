# html/semantics/embedded-content/the-img-element/update-the-image-data/not-broken-while-load-task-scheduled.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-the-image-data/not-broken-while-load-task-scheduled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Image shouldn't be broken while load task is scheduled.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
async function run_test(prop) {
  let img = new Image();
  img.width = 50;
  img.height = 50;
  img.alt = "Anything non-empty";
  img[prop] = `/images/green.png?not-broken-while-load-task-scheduled-` + Math.random();
  let load = new Promise(r => img.addEventListener("load", r, { once: true }));
  document.body.appendChild(img);

  assert_equals(img.clientWidth, 50, "clientWidth");
  assert_equals(img.clientHeight, 50, "clientHeight");
  assert_equals(getComputedStyle(img).height, "50px", "Computed height");
  assert_equals(getComputedStyle(img).width, "50px", "Computed height");

  await load;

  assert_equals(img.clientWidth, 50, "clientWidth");
  assert_equals(img.clientHeight, 50, "clientHeight");
  assert_equals(getComputedStyle(img).height, "50px", "Computed height");
  assert_equals(getComputedStyle(img).width, "50px", "Computed height");
}

promise_test(() => run_test("src"));
promise_test(() => run_test("srcset"));
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-the-image-data/not-broken-while-load-task-scheduled.html"
}
```
