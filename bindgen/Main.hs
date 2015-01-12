{-# LANGUAGE RecordWildCards #-}

import Options.Applicative
import qualified Language.C as LangC

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
    Options { .. } <- execParser handleOptions
    putStrLn $ headerFilename ++ " -> " ++ outputFilename
    headerContent <- LangC.parseCFilePre headerFilename
    print headerContent
