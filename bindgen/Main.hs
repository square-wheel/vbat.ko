import Options.Applicative

data Options = Options
    { headerFilename :: String
    , outputFilename :: String
    }

pOptions :: Parser Options
pOptions = Options
    <$> strArgument (metavar "SOURCE.H"  <> help "Source C header")
    <*> strArgument (metavar "TARGET.RS" <> help "Target Rust module")

handleOptions :: ParserInfo Options
handleOptions = info (helper <*> pOptions) (header "rust-bindgen")

main :: IO ()
main = do
    _opts <- execParser handleOptions
    putStrLn "So..."
