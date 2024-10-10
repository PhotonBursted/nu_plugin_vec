def main [file: path] {
    let raw_ds = open $file
    print "The input data we'll be working with:"
    print ($raw_ds | table)
    let people = $raw_ds | reject task | columns
    let whole_ds = $raw_ds | update cells -c $people {not ($in == "") | into int}
    let people_ds = $whole_ds | reject task
    let task_ds = $whole_ds | transpose -r
    
    let tasks = $task_ds | columns
    print "Which are the 3 most staffed tasks?"
    print (
        $tasks | each {|task|
            $task_ds | get $task | {task: $task, magnitude: ($in | vec magnitude)}
        } |
        sort-by magnitude | last 3 |
        table
    )
    print "Seems that DJ-ing is the most staffed task. Wait... isn't it a one-person job normally?"

    let name = "Maya"
    print $"Who tends to work on the same tasks than ($name)?"
    let hubert_vec = $people_ds | get $name
    print (
        $people | where {$in != $name} | each {|person|
            $people_ds | get $person | {person: $person, similarity: ($in | vec cos $hubert_vec)}
        } |
        sort-by similarity | last 3 |
        table
    )
}
