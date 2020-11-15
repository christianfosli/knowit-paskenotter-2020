open System.Net.Http

// This file contains numbers from 1 to 100_000, except for one number. Which number is missing?
// https://gist.githubusercontent.com/knowitkodekalender/f962392f90e181377dd51de6abe3b480/raw/c0ef3c2762d3f7bf9016b06d2d6b1fdce58f36cd/numbers.txt

let fetchNumbers () =
    async {
        let client = new HttpClient()

        let! res =
            client.GetAsync
                ("https://gist.githubusercontent.com/knowitkodekalender/f962392f90e181377dd51de6abe3b480/raw/c0ef3c2762d3f7bf9016b06d2d6b1fdce58f36cd/numbers.txt")
            |> Async.AwaitTask

        return! res.Content.ReadAsStringAsync() |> Async.AwaitTask
    }

fetchNumbers ()
|> Async.RunSynchronously
|> fun content -> content.Split(",")
|> Seq.map int
|> Set.ofSeq
|> Set.difference (Set.ofSeq [ 1 .. 100_000 ])
|> printfn "%A"
