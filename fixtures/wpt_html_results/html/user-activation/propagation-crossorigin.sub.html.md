# html/user-activation/propagation-crossorigin.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/propagation-crossorigin.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta name="timeout" content="long">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <script src="resources/utils.js"></script>
  <script>
    // Frame layout:
    // top=origin0:this-file [
    //   child1=origin1:child-one.html,
    //   child-xo=origin2:propagation-crossorigin-child.html [
    //     gchild=origin3:child-two.html
    //   ]
    // ]
    let propagation_test = async_test("Propagation test");

    let num_children_to_load = 3;
    let num_children_to_report = 3;

    function finishLoadPhase() {
        assert_equals(num_children_to_load, 0);

        test(() => {
            assert_false(navigator.userActivation.isActive);
            assert_false(navigator.userActivation.hasBeenActive);
        }, "Parent frame initial state");

        delayByFrames(() => test_driver.click(document.getElementById("child-xo")), 7);
    }

    function finishReportPhase() {
        assert_equals(num_children_to_report, 0);

        test(() => {
            assert_true(navigator.userActivation.isActive);
            assert_true(navigator.userActivation.hasBeenActive);
        }, "Parent frame final state");

        propagation_test.done();
    }

    window.addEventListener("message", event => {
        // Test driver can send messages too...
        if (typeof event.data !== "string") return;

        var msg = JSON.parse(event.data);

        if (msg.type == 'child-one-loaded') {
            test(() => {
                assert_false(msg.isActive);
                assert_false(msg.hasBeenActive);
            }, "Child1 frame initial state");
        } else if (msg.type == 'child-crossorigin-loaded') {
            test(() => {
                assert_false(msg.isActive);
                assert_false(msg.hasBeenActive);
            }, "Child2 frame initial state");
        } else if (msg.type == 'child-two-loaded') {
            test(() => {
                assert_false(msg.isActive);
                assert_false(msg.hasBeenActive);
            }, "Grandchild frame initial state");
        } else if (msg.type == 'child-one-report') {
            test(() => {
                assert_false(msg.isActive);
                assert_false(msg.hasBeenActive);
            }, "Child1 frame final state");
        } else if (msg.type == 'child-crossorigin-report') {
            // This msg was triggered by a user click.
            test(() => {
                assert_true(msg.isActive);
                assert_true(msg.hasBeenActive);
            }, "Child2 frame final state");

            // Ask remaining frames to report states.
            let ask_report = JSON.stringify({"type": "report"});
            frames[0].postMessage(ask_report, "*");
            frames[1].frames[0].postMessage(ask_report, "*");
        } else if (msg.type == 'child-two-report') {
            test(() => {
                assert_false(msg.isActive);
                assert_false(msg.hasBeenActive);
            }, "Grand child frame final state");
        }

        // Phase switching.
        if (msg.type.endsWith("-loaded")) {
            if (--num_children_to_load == 0)
                finishLoadPhase();
        } else if (msg.type.endsWith("-report")) {
            if (--num_children_to_report == 0)
                finishReportPhase();
        }
    });
    async function createIframes() {
        const child1 = document.createElement("iframe");
        child1.src = "http://{{hosts[alt][]}}:{{ports[http][0]}}/html/user-activation/resources/child-one.html";
        child1.id = "child1";
        document.body.appendChild(child1);
        await new Promise((resolve) => {
            child1.onload = resolve;
            document.body.appendChild(child1);
        });
        const childXO = document.createElement("iframe");
        childXO.id = "child-xo";
        childXO.src = "http://{{hosts[alt][]}}:{{ports[http][1]}}/html/user-activation/resources/propagation-crossorigin-child.sub.html";
        document.body.appendChild(childXO);
    }
  </script>
</head>
<body onload="createIframes()">
  <h1>User activation propagation across cross-origin frame boundary</h1>
  <p>Tests that user activation does not propagate across cross-origin frame boundary.</p>
  <ol id="instructions">
    <li>Click anywhere on the green area (child frame).
  </ol>
</body>
</html>
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
  "source_name": "html/user-activation/propagation-crossorigin.sub.html"
}
```
