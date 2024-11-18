<?php
// Example PHP script with class, while loop, and if statement

class Greeter {
    private $names;

    public function __construct($names) {
        $this->names = $names;
    }

    public function greetAll() {
        $index = 0;
        while ($index < count($this->names)) {
            $name = $this->names[$index];
            if (!empty($name)) {
                echo $this->greet($name) . "
";
            }
            $index++;
        }
    }

    private function greet($name) {
        return "Hello, " . $name . "!";
    }
}

$names = ["Alice", "Bob", "Charlie"];
$greeter = new Greeter($names);
$greeter->greetAll();
?>
