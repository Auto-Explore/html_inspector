# html/dom/elements/global-attributes/cdata-dir_auto.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/cdata-dir_auto.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
    <meta charset="utf-8">
    <link rel="author" title="Vincent Hilla" href="mailto:vhilla@mozilla.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/dom.html#the-directionality">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <div id="test1" dir="auto">
        <![CDATA[foo]]>اختبر
    </div>

    <iframe src="cdata-iframe.xhtml"></iframe>

    <script>
        function awaitMessage(msg) {
            return new Promise(res => {
                function waitAndRemove(e) {
                    if (e.data != msg)
                        return;
                    window.removeEventListener("message", waitAndRemove);
                    res();
                }
                window.addEventListener("message", waitAndRemove);
            });
        }

        const subframeLoaded = awaitMessage("subframe-loaded");

        async function createXHTMLCase(id) {
            await subframeLoaded;

            let iframe = document.querySelector("iframe");
            iframe.contentWindow.postMessage(id, "*");

            await awaitMessage(id);

            const doc = iframe.contentDocument;
            const div = doc.getElementById(id);
            const cdata = div.firstChild;

            return [div, cdata];
        }

        test(function() {
            const div = document.getElementById("test1");
            assert_true(div.matches(":dir(rtl)"));
        }, "Content of CDATA is ignored for dir=auto in html document");

        promise_test(async function() {
            let [div, cdata] = await createXHTMLCase(1);
            assert_true(div.matches(":dir(ltr)"));
        }, "Text in CDATASection is considered when determining auto directionality");

        promise_test(async function() {
            let [div, cdata] = await createXHTMLCase(2);
            assert_true(div.matches(":dir(ltr)"));
            cdata.remove();
            assert_true(div.matches(":dir(rtl)"));
        }, "Directionality is updated when removing CDATASection");

        promise_test(async function() {
            let [div, cdata] = await createXHTMLCase(3);
            assert_true(div.matches(":dir(ltr)"));
            cdata.textContent = "اختبر";
            assert_true(div.matches(":dir(rtl)"));
        }, "Directionality is updated when changing text of CDATASection");
    </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.bogus_comment",
      "message": "Bogus comment.",
      "severity": "Warning",
      "span": {
        "byte_end": 396,
        "byte_start": 394,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 2469,
        "byte_start": 2462,
        "col": 1,
        "line": 71
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
  "source_name": "html/dom/elements/global-attributes/cdata-dir_auto.html"
}
```
