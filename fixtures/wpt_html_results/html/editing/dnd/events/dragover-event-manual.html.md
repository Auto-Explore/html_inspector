# html/editing/dnd/events/dragover-event-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/dragover-event-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
    <head>
        <title>HTML5 Drag and Drop: Fire dragover event during the drag and drop processing</title>
        <meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/"/>
        <link rel="help" href="http://dev.w3.org/html5/spec/dnd.html#drag-and-drop-processing-model"/>
        <meta name="assert" content="Fire dragover event during the drag and drop processing"/>
        <script type="text/javascript">
            var EVENT, TARGET;

            function DragoverEvent(evt)
            {
                if ((TARGET == evt.target) && (EVENT == evt.type))
                {
                  document.getElementById("test_result").firstChild.data = "PASS";
                }
                else
                {
                  document.getElementById("test_result").firstChild.data = "FAIL";
                }
            }

            EVENT = "dragover";

            window.onload = function()
            {
                TARGET = document.getElementById("target");
                TARGET.addEventListener(EVENT, DragoverEvent, false);
            }
        </script>
    </head>
    <body>
        <pre>Description: Fire dragover event during the drag and drop processing</pre>
        <table id='testtable' border='1'>
            <tr>
                <td>Test Result</td>
                <td>Test Assertion</td>
            </tr>
            <tr>
                <td id='test_result'>Manual</td>
                <td id='test_assertion'>Test passes if if the word "PASS" appears to the left after following the steps below.
                <div id="manualsteps">
                    Steps:
                    <ol>
                        <li> Select the text inside the red box.
                        <li> Drag it, hover over the green box and then release the mouse
                    </ol>
                </div>
            </td>
            </tr>
        </table>
        <p>
        http://dev.w3.org/html5/spec/dnd.html#drag-and-drop-processing-model
        </p>
        <p>
        If the current target element is a DOM element, then fire a DND event named dragover at this current target element
        </p>
        <div style="border:2px red solid; width:100px">SampleText</div>
        <br /><br />
        <div id="target" style="border:2px green solid; width:200px; height:100px"></div>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 529,
        "byte_start": 498,
        "col": 9,
        "line": 9
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
  "source_name": "html/editing/dnd/events/dragover-event-manual.html"
}
```
