using System;
using System.Linq.Expressions;
using System.Net.Http;
using System.Text;
using System.Threading;
using System.Xml;
using System.IO;
using System.Diagnostics;

public class CPHInline
{
	public bool Execute()
	{
		// your main code goes here
		CPH.TryGetArg("messageStripped", out string message);
        long unixtime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
        string path = "C:/Users/とりです/Desktop/ずんだもんわーるど/tts_queue_Rust";
		string pathOfText = $"C:/Users/とりです/Desktop/ずんだもんわーるど/tts_queue_Rust/twitch_{unixtime}_10.txt";
        File.WriteAllText(pathOfText, message);

		return true;
	}
}