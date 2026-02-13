# html/semantics/scripting-1/the-script-element/css-module/css-module-worker-test.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/css-module/css-module-worker-test.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<head>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/common/utils.js"></script>
</head>

<body>
    <script>
        setup({allow_uncaught_exception: true});
        promise_test(function (test) {
            const uuid = token();
            const worker = new Worker(`./resources/worker.sub.js?key=${uuid}`, {
                type: "module"
            });
            return new Promise((resolve, reject) => {
                worker.addEventListener("error", resolve);
                worker.addEventListener("message", reject);
            }).then(async () => {
                const fetchResponse = await fetch(`./resources/record-fetch.py?key=${uuid}&action=getCount`);
                const fetchData = await fetchResponse.json();
                assert_equals(fetchData.count, 0, "Shouldn't have tried fetching CSS module in worker");
            });
        }, "A static import CSS Module within a web worker should not load and should not attempt to fetch the module.");

        promise_test(function (test) {
            const uuid = token();
            const worker = new Worker(`./resources/worker-dynamic-import.sub.js?key=${uuid}`, {
                type: "module"
            });

            return new Promise(resolve => {
                worker.addEventListener("message", resolve);
            }).then(async (event) => {
                assert_equals(event.data, "NOT LOADED");
                const fetchResponse = await fetch(`./resources/record-fetch.py?key=${uuid}&action=getCount`);
                const fetchData = await fetchResponse.json();
                assert_equals(fetchData.count, 0, "Shouldn't have tried fetching CSS module in worker");
            });
        }, "A dynamic import CSS Module within a web worker should not load and should not attempt to fetch the module.");

        promise_test(function (test) {
            const worker = new Worker("./resources/basic.css", {
                type: "module"
            });
            return new Promise(resolve => {
                worker.onerror = resolve;
            });
        }, "An attempt to load a CSS module as a worker should fail.");

    </script>

</body>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/scripting-1/the-script-element/css-module/css-module-worker-test.html"
}
```
